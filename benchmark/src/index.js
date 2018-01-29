/* eslint-disable no-console */

import { Suite } from 'benchmark';
import juice from 'juice';
import eyeliner from 'eyeliner';
import { readFileSync } from 'fs';
import { resolve as resolvePath } from 'path';

const createSuite = (title) => {
  const suite = new Suite();

  suite.on('cycle', (event) => {
    console.log(`${event.target} - ${title}`);
  });

  suite.on('complete', () => {
    console.log(`Fastest is ${suite.filter('fastest').map('name')} - ${title}`);
  });

  return {
    add: (...args) => suite.add(...args),
    run: () => new Promise((resolve) => {
      suite.run({ async: true });
      suite.on('complete', resolve);
    }),
  };
};

(async () => {
  // Small HTML + CSS
  const smallHtmlCss = `
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

  console.log(`## Test HTML: ${smallHtmlCss}`);
  console.log(`### Juice:\n${juice(smallHtmlCss)}`);
  console.log(`### Eyeliner:\n${eyeliner({ html: smallHtmlCss })}`);

  const smallSuite = createSuite('small HTML + CSS');

  smallSuite.add('Juice', () => {
    juice(smallHtmlCss);
  });

  smallSuite.add('Eyeliner', () => {
    eyeliner({ html: smallHtmlCss });
  });

  await smallSuite.run();

  // Large HTML + CSS
  const largeHtml = readFileSync(resolvePath(__dirname, '../src/bootstrap/docs/4.0/examples/navbars/index.html'), { encoding: 'utf8' });
  const largeCss = readFileSync(resolvePath(__dirname, '../src/bootstrap/dist/css/bootstrap.min.css'), { encoding: 'utf8' });

  const largeSuite = createSuite('large HTML + CSS');

  largeSuite.add('Juice', () => {
    juice.inlineContent(largeHtml, largeCss);
  });

  largeSuite.add('Eyeliner', () => {
    eyeliner({ html: largeHtml, css: largeCss });
  });

  await largeSuite.run();
})();
