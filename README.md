# TODO checker in files

### Example

> **NOTE** You must use **TODO:** in your comments.
> Multiline todo comment is not supported yet.

- Suppose you have a file like this:

```go
package main

import (
	"fmt"
	"net/http"
)

func homeHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Welcome to the home page!")
}

func aboutHandler(w http.ResponseWriter, r *http.Request) {
	// TODO: Add a detailed about page content
	fmt.Fprintf(w, "This is the about page.")
}

func main() {
	http.HandleFunc("/", homeHandler)
	http.HandleFunc("/about", aboutHandler)

	// TODO: Implement a 404 page for unknown routes

	fmt.Println("Server starting on http://localhost:8080")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		fmt.Println("Error starting server:", err)
	}
}
```

- Just run the binary `./todos-left ./src/main.go` and it will output:

```text
2 todos left in file "main.go":

line 13: Add a detailed about page content
line 21: Implement a 404 page for unknown routes
```
TEST branch
