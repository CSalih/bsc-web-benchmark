import { marked } from "marked";

export default (markdown?: string): string => {
  if (!markdown) {
    return "";
  }
  return marked(markdown, {
    async: false,
  });
};
