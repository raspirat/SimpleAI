"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
var fetch_html_1 = require("../../../scripts/ts/fetch_html");
var MainHeader = /** @class */ (function (_super) {
    __extends(MainHeader, _super);
    function MainHeader() {
        var _this = _super.call(this) || this;
        (0, fetch_html_1.fetch_html)(_this, '../html/index.html');
        return _this;
    }
    return MainHeader;
}(HTMLElement));
customElements.define('main_header', MainHeader);
