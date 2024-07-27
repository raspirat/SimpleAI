import * as fs from 'fs';

function _check_json(jsobj: JSON, var_prefix: string = "", initial_prefix: string = "")
{
    let prefix = initial_prefix;
    for (let key in jsobj)
    {
        let value = jsobj[key];
        let syntax: string = "*";
        if (typeof value === "object")
        {
            prefix = prefix + key + "-";
            _check_json(value, var_prefix, prefix);
        }
        else
        {
            switch (key) {
                case "color":
                    syntax = "<color>";
                    break;
                case "length":
                    syntax = "<length>";
                    break;
                case "number":
                    syntax = "<number>";
                    break;
                case "percentage":
                    syntax = "<percentage>";
                    break;
                case "length-percentage":
                    syntax = "<length-percentage>";
                    break;
                case "image":
                    syntax = "<image>";
                    break;
                case "url":
                    syntax = "<url>";
                    break;
                case "integer":
                    syntax = "<integer>";
                    break;
                case "angle":
                    syntax = "<angle>";
                    break;
                case "time":
                    syntax = "<time>";
                    break;
                case "resolution":
                    syntax = "<resolution>";
                    break;
                case "transform-function":
                    syntax = "<transform-function>";
                    break;
                case "custom-ident":
                    syntax = "<custom-ident>";
                    break;
                default:
                    syntax = "*"
                    break;
            }
            let name: string =  "--" + var_prefix + prefix + key;
            CSS.registerProperty(
                {
                    name: name,
                    syntax: syntax,
                    inherits: false,
                    initialValue: value
                }
            );
            console.log("registered css property:", name);
        }
        prefix = initial_prefix;
    }
}

export function reg_css(part: string, var_prefix: string = "")
{
    let editor_path: String = "@config/editor.json";
    let editor: JSON;
    if (var_prefix.length > 0)
    {
        var_prefix = var_prefix + "-"
    }
    fetch(editor_path.toString())
        .then(response => response.json())
        .then(editor => {
            let themes = editor["theme"]
            let theme_path: String = "@theme/" + themes["active_theme"] + "/init.json";
            fetch(theme_path.toString())
                .then(response => response.json())
                .then(data => {
                    _check_json(data[part], var_prefix);
                })
                .catch(error => console.error('Error fetching JSON:', error));
        })
        .catch(error => console.error('Error fetching JSON:', error));
}