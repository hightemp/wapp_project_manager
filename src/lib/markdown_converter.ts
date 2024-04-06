import { marked } from "marked"

export function convertMarkdownToHTML(text: string) {
  return marked.parse(text);
}
