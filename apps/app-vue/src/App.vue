<script setup>
import { ref, shallowRef } from "vue";
import { buildData } from "./data";

const selected = ref();
const rows = shallowRef([]);

function setRows(update = rows.value.slice()) {
  rows.value = update;
}

function add() {
  rows.value = rows.value.concat(buildData(1000));
}

function remove(id) {
  rows.value.splice(
    rows.value.findIndex((d) => d.id === id),
    1,
  );
  setRows();
}

function select(id) {
  selected.value = id;
}

function run() {
  if (window.afterFrame) {
    performance.mark("btn:run_start");
    document.getElementById("run").click();
    window.afterFrame(() => {
      performance.mark("btn:run_end");
    });
  }

  setRows(buildData());
  selected.value = undefined;
}

function update() {
  const _rows = rows.value;
  for (let i = 0; i < _rows.length; i += 10) {
    _rows[i].label += " !!!";
  }
  setRows();
}

function runLots() {
  if (window.afterFrame) {
    performance.mark("btn:run_start");
    document.getElementById("run").click();
    window.afterFrame(() => {
      performance.mark("btn:run_end");
    });
  }

  setRows(buildData(10000));
  selected.value = undefined;
}

function clear() {
  setRows([]);
  selected.value = undefined;
}

function swapRows() {
  const _rows = rows.value;
  if (_rows.length > 998) {
    const d1 = _rows[1];
    const d998 = _rows[998];
    _rows[1] = d998;
    _rows[998] = d1;
    setRows();
  }
}

// TODO: Evaluation purpose only!
if (typeof __pw_recorderState === "undefined") {
  // Source: https://github.com/andrewiggins/afterframe
  let callbacks = [];
  let channel = new MessageChannel();
  let postMessage = function () {
    this.postMessage(undefined);
  }.bind(channel.port2);
  channel.port1.onmessage = () => {
    let toFlush = callbacks;
    callbacks = [];
    let time = performance.now();
    for (let i = 0; i < toFlush.length; i++) {
      toFlush[i](time);
    }
  };
  channel = null;
  // @ts-ignore
  window.afterFrame = function (callback) {
    if (callbacks.push(callback) === 1) {
      requestAnimationFrame(postMessage);
    }
  };

  // Observe performance metrics
  const observer = new PerformanceObserver((list) => {
    list.getEntries().forEach((entry) => {
      if (entry.name === "btn:run_end") {
        const measure = performance.measure(
          "btn:run_duration",
          "btn:run_start",
          "btn:run_end",
        );
        console.log("Run time: ", measure.duration);
      }
    });
  });
  observer.observe({ entryTypes: ["mark"] });
}
</script>

<template>
  <div class="container">
    <div class="jumbotron">
      <div class="row">
        <div class="col-md-6">
          <h1>Vue App</h1>
        </div>
        <div class="col-md-6">
          <div class="row">
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="run"
                @click="run"
              >
                Create 1,000 rows
              </button>
            </div>
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="runlots"
                @click="runLots"
              >
                Create 10,000 rows
              </button>
            </div>
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="add"
                @click="add"
              >
                Append 1,000 rows
              </button>
            </div>
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="update"
                @click="update"
              >
                Update every 10th row
              </button>
            </div>
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="clear"
                @click="clear"
              >
                Clear
              </button>
            </div>
            <div class="col-sm-6 smallpad">
              <button
                type="button"
                class="btn btn-primary btn-block"
                id="swaprows"
                @click="swapRows"
              >
                Swap Rows
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <table class="table table-hover table-striped test-data">
      <tbody>
        <tr
          v-for="{ id, label } of rows"
          :key="id"
          :class="{ danger: id === selected }"
          :data-label="label"
          v-memo="[label, id === selected]"
        >
          <td class="col-md-1">{{ id }}</td>
          <td class="col-md-4">
            <a @click="select(id)">{{ label }}</a>
          </td>
          <td class="col-md-1">
            <a @click="remove(id)">
              <span
                class="glyphicon glyphicon-remove"
                aria-hidden="true"
              ></span>
            </a>
          </td>
          <td class="col-md-6"></td>
        </tr>
      </tbody>
    </table>
    <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true" />
  </div>
</template>
