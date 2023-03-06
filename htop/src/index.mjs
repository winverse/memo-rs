import { h, Component, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html`
    <div>
      ${props.cpus.map((cpu) => {
        return html` <div>${cpu.toFixed(2)}% usage</div> `;
      })}
    </div>
  `;
}

let i = 0;

setInterval(async () => {
  let res = await fetch("/api/cpus");

  if (res.status !== 200) {
    throw new Error(`HTTP error status: ${res.status}`);
  }

  let json = await res.json();
  render(html`<${App} cpus=${json}></${App}>`, document.body);
}, 1000);
