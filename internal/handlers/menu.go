package handlers

import (
	"casabaldini/internal/models"
	"html/template"
	"log"
	"net/http"
)

var tmps = template.Must(template.ParseGlob("templates/*.html"))

func Menu(w http.ResponseWriter, r *http.Request) {

	menus, err := models.GetMenus()
	if err != nil {
		log.Println(err)
		http.Error(w, "Errore interno", 500)
		return
	}

	err = tmps.ExecuteTemplate(w, "layout.html", map[string]interface{}{
		"Menu": menus,
	})
	if err != nil {
		log.Println(err)
	}
}
