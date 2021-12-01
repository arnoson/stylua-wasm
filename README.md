# StyLua Wasm

## Install

```
npm i stylua-wasm
```

## Usage

```ts
import init, {
  format,
  LineEndings,
  IdentType,
  QuoteStyle,
  OutputVerification,
} from 'stylua-wasm'

init().then(() => {
  const config = {
    column_width: 80,
    line_endings: LineEndings.Unix,
    ident_width: 2,
    ident_type: IdentType.Spaces,
    quote_style: QuoteStyle.AutoPreferSingle,
    no_call_parentheses: false,
  }

  const formatted = format(
    'print(  "hello"  )',
    config,
    OutputVerification.Full
  )
})
```

If you use a bundler, you have to make sure to pass the correct URL to `init()`.
In [vite](https://vitejs.dev/) this would look like this:

```ts
import init, { format, OutputVerification } from 'stylua-wasm'
import wasmUrl from 'stylua-wasm/stylua_wasm_bg.wasm?url'

init(wasmUrl).then(() =>
  console.log(
    format(
      'print( "hello" )',
      {}, // Using default options
      OutputVerification.Full
    )
  )
)
```

### API

```ts
interface Config {
  column_width?: number
  line_endings?: LineEndings
  indent_type?: IndentType
  indent_width?: number
  quote_style?: QuoteStyle
  no_call_parentheses?: boolean
}

function format(
  input: string,
  config: Config,
  output_verification?: OutputVerification
): string
```

## Caveats

- Range is currently not supported, so you can only format the whole text.
