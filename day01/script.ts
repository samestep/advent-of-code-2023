import example from "./example.txt?raw";
import input from "./input.txt?raw";

const parseChar = (c: string): number | undefined => {
  // switch for all digits
  switch (c) {
    case "0":
      return 0;
    case "1":
      return 1;
    case "2":
      return 2;
    case "3":
      return 3;
    case "4":
      return 4;
    case "5":
      return 5;
    case "6":
      return 6;
    case "7":
      return 7;
    case "8":
      return 8;
    case "9":
      return 9;
    default:
      return undefined;
  }
};

const parse = (line: string): { num: number; html: HTMLElement } => {
  const digits = [...line].map(parseChar);

  let i = 0;
  while (digits[i] === undefined) ++i;

  let j = line.length - 1;
  while (digits[j] === undefined) --j;

  const num = digits[i]! * 10 + digits[j]!;
  const html = document.createElement("div");

  const a = document.createElement("span");
  a.innerText = line.substring(0, i);
  html.appendChild(a);
  const b = document.createElement("span");
  b.innerText = line.substring(i, i + 1);
  b.style.fontWeight = "bold";
  b.style.color = "red";
  html.appendChild(b);
  if (i === j) {
    b.style.color = "purple";
  } else {
    const c = document.createElement("span");
    c.innerText = line.substring(i + 1, j);
    html.appendChild(c);
    const d = document.createElement("span");
    d.innerText = line.substring(j, j + 1);
    d.style.fontWeight = "bold";
    d.style.color = "blue";
    html.appendChild(d);
  }
  const e = document.createElement("span");
  e.innerText = line.substring(j + 1);
  html.appendChild(e);

  return { num, html };
};

const set = (str: string) => {
  const root = document.getElementById("content")!;
  root.innerHTML = "";
  const grid = document.createElement("div");
  grid.style.fontFamily = "monospace";
  grid.style.display = "grid";
  grid.style.gridTemplateColumns = "2";
  grid.style.gap = "0 1em";
  root.appendChild(grid);

  let sum = 0;

  const lines = str.split("\n");
  lines.pop(); // the last line is empty
  for (let i = 0; i < lines.length; ++i) {
    const soFar = document.createElement("div");
    soFar.style.gridRow = `${i + 1}`;
    soFar.style.gridColumn = "1";
    soFar.style.textAlign = "right";
    soFar.style.fontWeight = "bold";
    grid.appendChild(soFar);
    soFar.innerText = `${sum} +`;

    const { num, html } = parse(lines[i]);
    sum += num;
    html.style.gridRow = `${i + 1}`;
    html.style.gridColumn = "2";
    grid.appendChild(html);
  }

  const total = document.createElement("div");
  total.style.gridRow = `${lines.length + 1}`;
  total.style.gridColumn = "1";
  total.style.textAlign = "right";
  total.style.fontWeight = "bold";
  grid.appendChild(total);
  total.innerHTML = `${sum}&nbsp;&nbsp;`;
};

document
  .getElementById("example")
  ?.addEventListener("change", () => set(example));
document.getElementById("input")?.addEventListener("change", () => set(input));

set(example);
