/* eslint-disable no-console */

import { Suite } from 'benchmark';
import juice from 'juice';
import eyeliner from 'eyeliner';

const fixture = `
  <!doctype html>
  <html>
    <head>
      <style>
        .red {
          color: red;
        }
      </style>
    </head>
    <body>
      <h1 class="red">This is red</h1>
    </body>
  </html>
`;

console.log(`## Test HTML: ${fixture}`);
console.log(`### Juice:\n${juice(fixture)}`);
console.log(`### Eyeliner:\n${eyeliner({ html: fixture })}`);

const suite = new Suite();

suite.add('Juice', () => {
  juice(fixture);
});

suite.add('Eyeliner', () => {
  eyeliner({ html: fixture });
});

suite.on('cycle', (event) => {
  console.log(`${event.target}`);
});

suite.on('complete', () => {
  console.log(`Fastest is ${suite.filter('fastest').map('name')}`);
});

suite.run({ async: true });
