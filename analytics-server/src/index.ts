import type { NextApiRequest, NextApiResponse } from "next";
import fs from "node:fs";

type ResponseData = {
  message: string;
};

const sendToMongoDB = (metric: string) => {
  const apiKey =
    "Y0aPnGiuH26L7jFRZfrGyopiu6FUSOSmp6zpqwcKejyqG4aQT0sJjL2TxVDliX7C";
  const url =
    "https://eu-central-1.aws.data.mongodb-api.com/app/data-rwbye/endpoint/data/v1/action/insertOne";

  const mongodbAPIAdapter = (metric: string) => {
    return {
      dataSource: "web-vital-analytics",
      database: "web-vital-analytics",
      collection: "web-vitals",
      document: JSON.parse(metric),
    };
  };

  fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "api-key": apiKey,
    },
    body: JSON.stringify(mongodbAPIAdapter(metric)),
  }).catch((error) => {
    // eslint-disable-next-line no-console
    console.error("Error while sending analytics to MongoDB:", error);
  });
};

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<ResponseData>
) {
  if (req.method !== "POST") {
    res.status(405).json({
      message: "Method Not Allowed",
    });
    return;
  }
  if (req.headers["content-type"]?.indexOf("application/json") !== -1) {
    res.status(415).json({
      message: "Media type not supported. Only application/json is supported.",
    });
    return;
  }

  if (req.body) {
    const date = new Date().toISOString().split("T")[0];
    const steam = fs.createWriteStream(`.web-vitals/web-vitals${date}.json`, {
      flags: "a",
    });
    steam.write(`${req.body}\n`);
    steam.end();

    sendToMongoDB(req.body);
  }

  res.status(204).end();
}
