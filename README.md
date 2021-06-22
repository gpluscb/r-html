# r-html

Shitty template engine for rust that is kinda cool actually (in concept, maybe).

## Templates:

```html
<!--args title: &str, data: &[CoolStruct] -->

<!--rs use crate::CoolStruct; -->

<!doctype html>
<html lang="en">
<head>
    <title>[rs title]</title>
</head>

<body>
<div class="container">
    <h1>[rs title]</h1>

    <table class="table">
        <thead>
        <tr>
            <th>Some name</th>
            <th>A datapoint</th>
            <th>Another number</th>
            <th>More numbers</th>
        </tr>
        </thead>
        <!--rs for data_row in data { -->
        <tr>
            <td>[rs data_row.name]<!--rs if let Some(alt_name) = &data_row.alternative_name {
                --> (aka [rs alt_name])<!--rs
            } --></td>
            <td>[rs data_row.datapoint.to_string()]</td>
            <td>[rs data_row.second_datapoint.to_string()]</td>
            <td>[rs data_row.another_number.to_string()]</td>
        </tr>
        <!--rs } -->
    </table>
</div>
</body>
</html>
```

`<!--args ... -->` defines the arguments your macro will take.
Stuff inside of `<!--rs ... -->` will literally appear in the generated rust code.
This is the "kinda cool" part: because that's an HTML comment you can still look at the template as an html file in your browser and it will look ok.
A rust expressions inside of `[rs ...]` will be evaluated and then inserted in the HTML.
That stuff needs to implement `AsRef<str>` rn.

## Your code:

```
let html: String = template!("/path/to/my_cool_template.rs.html", "title", &data);
```
`Cargo.toml`:
```
r-html = { git = "https://github.com/gpluscb/r-html.git", version = "0.1.0" }
```


## Cool?

This is shitty because I wanted it to work and nothing else.
Error messages are bad.
Edge cases are bad.
No escaping the control sequences.

But maybe it's good enough.
For me it is.
