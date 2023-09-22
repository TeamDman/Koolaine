console.log("[Konvord] popup.js loaded")
const input = document.getElementById("server-url");
const defaultServerUrl = "https://localhost:5876/";
input.setAttribute("placeholder", defaultServerUrl);

function toast(message) {
    const holder = document.getElementById("toast-holder");
    const toast = document.createElement("div");
    toast.classList.add("toast");
    toast.innerText = message;

    holder.appendChild(toast);
    setTimeout(() => {
        toast.remove();
    }, 3000);
}

input.addEventListener("change", function () {
    console.log("[Konvord] server-url changed")
    const url = input.value;
    chrome.storage.local.set({ serverUrl: url }, function () {
        console.log("[Konvord] Server URL is set to " + url);
        toast("Server URL is set to " + url);
    });
});

chrome.storage.local.get("serverUrl", function (data) {
    input.value = data.serverUrl || defaultServerUrl;
});
