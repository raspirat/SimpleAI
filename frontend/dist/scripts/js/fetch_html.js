"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.fetch_html = void 0;
function fetch_html(self, path) {
    var PATH_URL = new URL(path);
    fetch(PATH_URL)
        .then(function (response) { return response.text(); })
        .then(function (data) {
        var TEMPLATE = document.createElement('template');
        var SHADOW_ROOT = self.attachShadow({ mode: 'open' });
        TEMPLATE.innerHTML = data;
        SHADOW_ROOT.appendChild(TEMPLATE.content.cloneNode(true));
    });
}
exports.fetch_html = fetch_html;
