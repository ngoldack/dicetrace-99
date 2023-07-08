package main

import (
	"log"
	"net/http"
	"strconv"
	"strings"
	"time"

	cache "github.com/chenyahui/gin-cache"
	"github.com/chenyahui/gin-cache/persist"
	"github.com/fzerorubigd/gobgg"
	"github.com/gin-gonic/gin"

	"schneider.vip/problem"
)

func health(c *gin.Context) {
	client := gobgg.NewBGGClient()
	start := time.Now()
	_, err := client.Hotness(c, 0)
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(
			problem.Title("Error while pinging bgg"),
			problem.Custom("error", err.Error())).WriteTo(c.Writer)
		return
	}
	delay := time.Since(start).String()

	c.JSON(http.StatusOK, gin.H{
		"status": http.StatusOK,
		"delay":  delay,
	})

}

func search(c *gin.Context) {
	q := c.Query("q")
	if q == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing query")).WriteTo(c.Writer)
		return
	}

	client := gobgg.NewBGGClient()
	results, err := client.Search(c, q, gobgg.SearchTypes(gobgg.BoardGameType))
	if err != nil {
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while searching")).WriteTo(c.Writer)
		return
	}

	log.Printf("Found %d results for %s", len(results), q)

	c.JSON(http.StatusOK, results)
}

func collection(c *gin.Context) {
	client := gobgg.NewBGGClient()
	if err := client.Login(c, "ngoldack", "Nicooo1999"); err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while logging in")).WriteTo(c.Writer)
		return
	}

	collection, err := client.GetCollection(c, client.GetActiveUsername())
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while getting collection")).WriteTo(c.Writer)
		return
	}

	c.JSON(http.StatusOK, collection)
}

func plays(c *gin.Context) {
	username := c.Query("username")
	if username == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing username")).WriteTo(c.Writer)
		return
	}

	options := make([]gobgg.PlaysOptionSetter, 0)
	options = append(options, gobgg.SetUserName(username))

	ids := strings.Split(c.Query("game_ids"), ",")
	for _, id := range ids {
		if i, err := strconv.Atoi(id); err == nil {
			options = append(options, gobgg.SetGameID(i))
		}
	}

	log.Println(len(options))

	client := gobgg.NewBGGClient()
	plays, err := client.Plays(c, options...)
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while getting plays")).WriteTo(c.Writer)
		return
	}

	c.JSON(http.StatusOK, plays)
}

func main() {
	log.Println("Starting server...")
	r := gin.Default()

	store := persist.NewMemoryStore(60 * time.Second)

	r.GET("/health", cache.CacheByRequestPath(store, time.Second), health)
	r.GET("/search", cache.CacheByRequestURI(store, time.Minute), search)
	r.GET("/collection", cache.CacheByRequestURI(store, time.Minute), collection)
	r.GET("/plays", cache.CacheByRequestURI(store, time.Minute), plays)
	r.Run()

	log.Println("Server stopped")
}
