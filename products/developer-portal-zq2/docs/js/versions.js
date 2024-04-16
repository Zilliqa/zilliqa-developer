function fillInVersionNumbers() {
  var ids = document.getElementsByClassName("zq2_vsn");
  for (let i = 0; i < ids.length; i++) {
    elem = ids[i];
    url = elem.innerHTML.trim();
    var data = {
      id: "1",
      jsonrpc: "2.0",
      method: "GetVersion",
      params: [""],
    };
    fetch("https://" + url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    })
      .then((response) => response.json())
      .then((data) => {
        elem.innerHTML = data["result"].Version;
        elem.zilliqaVerison = data["result"].Version;
        fillInDocumentLinks(elem.attributes["id"].value, data["result"]);
      });
  }
}

function fillInDocumentLinks(version_id, vsn_obj) {
  console.log(JSON.stringify(vsn_obj));
  var ids = document.getElementsByClassName("zq2_docs_" + version_id);
  for (let i = 0; i < ids.length; i++) {
    let elem = ids[i];
    elem.innerHTML = `<a href="https://github.com/zilliqa/zq2/blob/${vsn_obj["Commit"]}/README.md#supported-apis">Supported APIs</a>`;
  }
}

document$.subscribe( function () {
  fillInVersionNumbers();
});
