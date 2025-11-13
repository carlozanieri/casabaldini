package main

import (
	"casabaldini/internal/db"
	"casabaldini/internal/handlers"
	"log"
	"net/http"
)

func main() {
	db.Init()

	http.HandleFunc("/", handlers.Home)
	http.HandleFunc("/menu", handlers.Menu)

	log.Println("Server avviato su http://localhost:8080")
	http.ListenAndServe(":8080", nil)
}
