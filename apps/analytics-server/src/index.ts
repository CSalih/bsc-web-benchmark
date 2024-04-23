import 'dotenv/config';
import express, {type Request, type Response} from "express";
import {sendToMongoDB} from "./sendToMongoDB";

const app = express();
const port = process.env.PORT || 8000;


app.use(express.json());
app.use(express.text());


app.get("/web-vital.js", (_: Request, res: Response) => {


  res.status(204).end();
});

app.post("/api/v1/event", (req: Request, res: Response) => {
  if (!req.body) {
    return res.status(422).json({
      message: "Body is required but missing",
    });
  }
  sendToMongoDB(req.body);

  res.status(204).end();
});


app.listen(port, () => {
  console.log(`[server]: Server is running at http://localhost:${port}`);
});
