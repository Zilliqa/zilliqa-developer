# zil <-> hex format converter

This is a quick demo of an in-browser Bech32 (`zil1...`) to hex (and back) converter.

Run the web server of your choice on the documents in `content/` and point your browser at `index.html`.

If you want server-side, just grab `content/js/chain.js` and use it (`buffer.js` is just a polyfill for node-js `buffer`).

If you are on Linux:

```sh
docker compose up
```

Should get you a copy of nginx on port 3000 which serves the content.

If you have problems please contact your Zilliqa support contact, or [richard@zilliqa.com](mailto:richard@zilliqa.com).
