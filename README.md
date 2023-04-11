# Proxrs

## What is Proxrs?

Proxrs is a powerful reverse-proxy server written in Rust, designed to provide secure access to websites and applications through user authentication. With Proxrs, you can protect your web applications from unauthorized access and keep your users safe.

Proxrs is built using Rust, a modern and efficient programming language known for its speed, safety, and reliability. It is designed to be highly configurable, with a range of options for customizing its behavior to fit your specific use case.

## Static files

Proxrs uses some static files to provide a way to log in and log out. You can find them in the `static` directory. You can customize them to fit your needs. but keep in mind that the forms can't change much like the action {{XXX_route}} is used to replace the route by the one you set in the configuration file.

## Configuration

Proxrs uses an `.env` file to configure itself. You can find an example in the `example.env` file. You can copy it and rename it to `.env` to use it. All the options are explained in the file.
