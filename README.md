# Hust

Hust is an HTML-first way to embed Rust in HTML.

## This Crate

This crate is for using hust in your projects.

## Usage

Use a .hust file like so:

```
let result = include_hust!("test.hust");
```

Imagine your file looks like:
```
<h1>Hello, World!</h1>
<p>Hello from Hust!</p>
<%= variable_name %>
```

The file will be converted to rust code that looks like the below, which will be inserted in place where the macro is run.
```
let output_buffer = String::new();
output_buffer.push_str("<h1>Hello, World!</h1>\n\r<p>Hello from Hust!</p>\n\r");
output_buffer.push_str(variable_name);
output_buffer
```

This code is then inlined right where you called the macro.

## Example Hust

```
<h1>User</h1>
<div class="user">
  <%= &user.username %>
</div>
```

Or for more complex usage:

```
<h1>All Users</h1>
<div class="py-2">
  <%= &users.len().to_string() %> users found.
</div>
<% for user in users { %>
  <div class="user">
    <a href="/users/<%= &user.id.to_string() %>">
    <%= &user.username %>
    </a>
  </div>
<% } %>

<a href="/users/new" class="btn btn-primary">New User</a>
```

## Downsides / Drawbacks

Currently, it is impossible to debug. Due to the way macros work in Rust currently, any errors in your Hust code will be shown at `include_hust!` and not on the line of the file that actually caused the error. We are thinking of ways to improve this in the future.
