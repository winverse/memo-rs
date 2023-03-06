import { h, Component, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html`
    <ul>
      ${props.cpus.map((cpu, index) => {
        return html`
          <li class="bar">
            <div class="bar-inner" style="width: ${cpu}%;"></div>
            <div class="bar-text">CPU ${index + 1}: ${cpu.toFixed(2)}%</div>
          </li>
        `;
      })}
    </ul>
  `;
}

let i = 0;

const update = async () => {
  let res = await fetch("/api/cpus");

  if (res.status !== 200) {
    throw new Error(`HTTP error status: ${res.status}`);
  }

  let json = await res.json();
  render(html`<${App} cpus=${json}></${App}>`, document.body);
};

update();
setInterval(update, 200);
