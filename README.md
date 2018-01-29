# node-eyeliner

Native bindings for [eyeliner][eyeliner], a CSS inliner for making emails.

## Example

```js
import eyeliner from 'eyeliner';

const html = `
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

const inlinedHtml = eyeliner({ html });

console.log(inlinedHtml);
// <!DOCTYPE html><html><head>
//
//       <style></style></head>
//       <body>
//         <h1 class="red" style="color: red;">This is red</h1>
//
//
//   </body></html>
```

## Benchmarks

`eyeliner` is ultra fast like 3x faster than the competition. Check the
[benchmark directory][benchmark] to see the comparisons.

[eyeliner]: https://github.com/dfrankland/eyeliner
[benchmark]: https://github.com/dfrankland/node-eyeliner/tree/master/benchmark
