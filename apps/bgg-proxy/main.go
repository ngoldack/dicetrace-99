package main

import (
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/chenyahui/gin-cache/persist"
	"github.com/fzerorubigd/gobgg"
	"github.com/gin-gonic/gin"
	"github.com/go-redis/redis/v8"
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

func health(c *gin.Context) {
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

	c.JSON(http.StatusOK, gin.H{
		"status": http.StatusOK,
		"delay":  delay,
	})

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

	client.mu.Lock()
	defer client.mu.Unlock()

	collection, err := client.bgg.GetCollection(c, client.bgg.GetActiveUsername())
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
	idString := c.Param("id")
	if idString == "" {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Missing id")).WriteTo(c.Writer)
		return
	}

	id, err := strconv.Atoi(idString)
	if err != nil {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Invalid id")).WriteTo(c.Writer)
		return
	}

	client.mu.Lock()
	defer client.mu.Unlock()

	item, err := client.bgg.GetThings(c, gobgg.GetThingIDs(int64(id)))
	if err != nil {
		log.Fatal(err)
		problem.Of(http.StatusInternalServerError).Append(problem.Title("Error while getting item")).WriteTo(c.Writer)
		return
	}

	if len(item) == 0 {
		problem.Of(http.StatusBadRequest).Append(problem.Title("Item not found")).WriteTo(c.Writer)
		return
	}

	c.JSON(http.StatusOK, item[0])
}

func main() {
	log.Println("Starting server...")

	// load .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	r := gin.Default()

	// setup redis
	opt, _ := redis.ParseURL(os.Getenv("REDIS_URL"))
	client := redis.NewClient(opt)

	defer client.Close()

	// cache for one week
	store := persist.NewRedisStore(client)

	cacheDuration := time.Hour * 24 * 7

	r.GET("/health", ginCache.CacheByRequestPath(store, cacheDuration), health)
	r.GET("/search", ginCache.CacheByRequestURI(store, cacheDuration), search)
	r.GET("/collection", ginCache.CacheByRequestURI(store, cacheDuration), collection)
	r.GET("/plays", ginCache.CacheByRequestURI(store, cacheDuration), plays)
	r.GET("/item/:id", ginCache.CacheByRequestURI(store, cacheDuration), item)
	r.Run()

	log.Println("Server stopped")
}
