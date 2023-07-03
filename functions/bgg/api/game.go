package handler

import (
	"fmt"
	"net/http"

	"github.com/fzerorubigd/gobgg"
)

func Handler(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")

	if name == "" {
		http.Error(w, "name is required", http.StatusBadRequest)
		return
	}

	bgg := gobgg.NewBGGClient(gobgg.SetClient(&http.Client{}))

	results, err := bgg.Search(r.Context(), name, gobgg.SearchTypes(gobgg.BoardGameType))
	if err != nil {
		http.Error(w, fmt.Sprintf("search failed: %s", err), http.StatusInternalServerError)
		return
	}

	if len(results) == 0 {
		fmt.Fprintf(w, "<h1>No results found</h1>")
		return
	}

	fmt.Fprintf(w, "<h1>Results: %v</h1>", len(results))
	fmt.Fprintf(w, "<ul>")
	for _, r := range results {
		fmt.Fprintf(w, "<li><h2>%v</h2> - %s</li>", r.ID, r.Name)
	}
	fmt.Fprintf(w, "</ul>")
	w.WriteHeader(http.StatusOK)
}
