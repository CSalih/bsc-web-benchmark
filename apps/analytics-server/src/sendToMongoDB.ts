if (!process.env.MONGODB_BASE_URL) {
  throw new Error("MONGODB_BASE_URL is not defined");
}
if (!process.env.MONGODB_API_KEY) {
  throw new Error("MONGODB_API_KEY is not defined");
}

const mongodbAPIAdapter = (metric: string) => {
  return {
    dataSource: "web-vital-analytics",
    database: "web-vital-analytics",
    collection: "bsc-web-benchmark",
    document: JSON.parse(metric),
  };
};

export const sendToMongoDB = (metric: string) => {
  const url = process.env.MONGODB_BASE_URL as string;
  const apiKey = process.env.MONGODB_API_KEY as string;

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
