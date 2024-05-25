const fs = require("node:fs")
const { execSync } = require("child_process")
const path = require("path")

function humanizeBytes(bytes, decimals = 2) {
    if (!+bytes) {
        return "0 Bytes"
    }

    const k = 1024
    const dm = decimals < 0 ? 0 : decimals
    const sizes = ["Bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"]

    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
}

const fileList = []

const directorySizeRecursive = (dir, totalSize) => {
    const files = fs.readdirSync(dir, {
        withFileTypes: true,
    })

    return files
        .map((file) => {
            const filePath = path.join(dir, file.name)

            if (file.isDirectory()) {
                return directorySizeRecursive(filePath)
            }

            if (file.isFile() && (file.name.endsWith(".js") || file.name.endsWith(".wasm"))) {
                const { size } = fs.statSync(filePath)
                if (file.name.endsWith(".js") || file.name.endsWith(".wasm")) {
                    // Ngnix defaults to level 1
                    const gzipSize = parseInt(execSync(`gzip -1 --stdout ${filePath} | wc --bytes`).toString().trim())
                    // Ngnix defaults to level 6
                    const brotliSize = parseInt(
                        execSync(`brotli -6 --stdout ${filePath} | wc --bytes`).toString().trim(),
                    )
                    fileList.push({
                        filePath,
                        size: size,
                        gzipSize: gzipSize,
                        brotliSize: brotliSize,
                    })
                }

                return size
            }

            return 0
        })
        .reduce((acc, size) => acc + size, 0)
}

const buildDirectorySizes = (directory) => {
    return fs
        .readdirSync(directory)
        .filter((filePath) => filePath.startsWith("app-"))
        .map((filePath) => path.join(directory, `${filePath}/dist`))
        .map((filePath) => {
            const size = fs.existsSync(filePath) ? directorySizeRecursive(filePath, 0) : -1
            return {
                filePath,
                size: size,
            }
        })
}

buildDirectorySizes("apps")

console.table(
    fileList.map((file) => ({
        project: file.filePath.split("/")[1],
        filePath: file.filePath,
        uncompressed: humanizeBytes(file.size),
        gzip: humanizeBytes(file.gzipSize),
        brotli: humanizeBytes(file.brotliSize),
    })),
)

const totalStats = fileList.reduce((acc, file) => {
    const project = file.filePath.split("/")[1]
    if (!acc[project]) {
        acc[project] = {
            uncompressed: 0,
            gzip: 0,
            brotli: 0,
        }
    }
    acc[project] = {
        uncompressed: acc[project].uncompressed + file.size,
        gzip: acc[project].gzip + file.gzipSize,
        brotli: acc[project].brotli + file.brotliSize,
    }
    return acc
}, {})

const totalStatsArray = Object.keys(totalStats).map((project) => ({
    project: project,
    uncompressed: humanizeBytes(totalStats[project].uncompressed),
    gzip: humanizeBytes(totalStats[project].gzip),
    gzipRatio: (totalStats[project].gzip / totalStats[project].uncompressed).toFixed(2),
    brotli: humanizeBytes(totalStats[project].brotli),
    brotliRatio: (totalStats[project].brotli / totalStats[project].uncompressed).toFixed(2),
}))
console.table(totalStatsArray)
