package main

import (
	"log"
	"net/http"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/chenyahui/gin-cache/persist"
	"github.com/fzerorubigd/gobgg"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"

	ginCache "github.com/chenyahui/gin-cache"

	"schneider.vip/problem"

	"go.uber.org/ratelimit"
)

type syncedClient struct {
	mu  sync.Mutex
	bgg *gobgg.BGG
}

var rl = ratelimit.New(10, ratelimit.Per(60*time.Second)) // creates a 10 per minutes rate limiter.

var httpClient = http.DefaultClient

var client = &syncedClient{
	mu:  sync.Mutex{},
	bgg: gobgg.NewBGGClient(gobgg.SetLimiter(rl), gobgg.SetClient(httpClient)),
}

func health(store *persist.MemoryStore) gin.HandlerFunc {
	return func(c *gin.Context) {
		client.mu.Lock()
		defer client.mu.Unlock()

		start := time.Now()
		_, err := client.bgg.Hotness(c, 0)
		if err != nil {
			log.Fatal(err)
			problem.Of(http.StatusInternalServerError).Append(
				problem.Title("Error while pinging bgg"),
				problem.Custom("error", err.Error())).WriteTo(c.Writer)
			return
		}
		delay := time.Since(start).String()

		keys := make([]string, 0, len(store.Cache.GetItems()))
		for k := range store.Cache.GetItems() {
			keys = append(keys, k)
		}

		c.JSON(http.StatusOK, gin.H{
			"cache":  gin.H{"keys": keys, "size": len(keys)},
			"status": http.StatusOK,
			"delay":  delay,
		})
	}

}

func search(c *gin.Context) {
	client.mu.Lock()
	defer client.mu.Unlock()

	q := c.Query("q")
	if q == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing query")).WriteTo(c.Writer)
		return
	}

	results, err := client.bgg.Search(c, q, gobgg.SearchTypes(gobgg.BoardGameType))
	if err != nil {
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while searching")).WriteTo(c.Writer)
		return
	}

	log.Printf("Found %d results for %s", len(results), q)

	c.JSON(http.StatusOK, results)
}

func collection(c *gin.Context) {
	username := c.Query("username")
	if username == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing username")).WriteTo(c.Writer)
		return
	}

	client.mu.Lock()
	defer client.mu.Unlock()

	collection, err := client.bgg.GetCollection(c, username)
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

	client.mu.Lock()
	defer client.mu.Unlock()

	plays, err := client.bgg.Plays(c, options...)
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while getting plays")).WriteTo(c.Writer)
		return
	}

	c.JSON(http.StatusOK, plays)
}

func item(c *gin.Context) {
	idsQuery := c.Query("ids")
	if idsQuery == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing id(s)")).WriteTo(c.Writer)
		return
	}

	idsStrings := strings.Split(idsQuery, ",")
	ids := make([]int64, 0)
	for _, idString := range idsStrings {
		id, err := strconv.Atoi(idString)
		if err != nil {
			problem.Of(http.StatusBadRequest).Append(problem.Title("Invalid id")).WriteTo(c.Writer)
			return
		}
		ids = append(ids, int64(id))
	}

	client.mu.Lock()
	defer client.mu.Unlock()

	items, err := client.bgg.GetThings(c, gobgg.GetThingIDs(ids...))
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while getting item")).WriteTo(c.Writer)
		return
	}

	if len(items) == 0 {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Item not found")).WriteTo(c.Writer)
		return
	}

	c.JSON(http.StatusOK, items)
}

func main() {
	log.Println("Starting server...")

	// load .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	// cache for one week
	cacheDuration := time.Hour * 24 * 7
	store := persist.NewMemoryStore(cacheDuration)

	r := gin.Default()
	r.GET("/health", ginCache.CacheByRequestPath(store, time.Second), health(store))

	v1 := r.Group("/api/v1")
	v1.GET("/search", ginCache.CacheByRequestURI(store, cacheDuration), search)
	v1.GET("/collection", ginCache.CacheByRequestURI(store, cacheDuration), collection)
	v1.GET("/plays", ginCache.CacheByRequestURI(store, cacheDuration), plays)
	v1.GET("/item", ginCache.CacheByRequestURI(store, cacheDuration), item)

	r.Run()

	log.Println("Server stopped")
}
