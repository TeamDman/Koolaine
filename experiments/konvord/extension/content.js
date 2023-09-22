const tag = `[Konvord]`
console.log(`${tag} content.js loaded`);
const defaultServerUrl = "https://localhost:5876/";
let serverUrl = defaultServerUrl;
chrome.storage.local.get("serverUrl", function (data) {
    serverUrl = data.serverUrl || defaultServerUrl;
    console.log(`${tag} serverUrl is ${serverUrl}`);
});
// listen for changes
chrome.storage.onChanged.addListener(function (changes, namespace) {
    for (let key in changes) {
        if (key === "serverUrl") {
            serverUrl = changes[key].newValue;
            console.log(`${tag} serverUrl is ${serverUrl}`);
        }
    }
});

// async function setup() {
//     console.log(`${tag} setup()`);
//     // create port input element
//     const portInput = document.createElement('input');
//     portInput.setAttribute('type', 'number');
//     portInput.setAttribute('placeholder', 'port');
//     portInput.setAttribute('value', '8080');
//     portInput.setAttribute('id', 'konvord-port');
//     // make it always top right minus 80 px
//     portInput.style.position = 'fixed';
//     portInput.style.top = '0';
//     portInput.style.right = '80px';
//     // make it #343541
//     portInput.style.backgroundColor = '#343541';
//     // add it to the body
//     document.body.appendChild(portInput);
// }
// setup();
// document.addEventListener("load", setup);