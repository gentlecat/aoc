package main

import (
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"path"
	"strings"
)

func main() {

	if len(os.Args) != 2 {
		fmt.Println("INVALID INPUT\nYear argument required.")
		return
	}

	year := os.Args[1]
	if len(year) != 4 {
		fmt.Printf("\"%s\" doesn't look like a year we support around here...", year)
	}

	day := 1

	for {
		input, exists, e := getInput(year, fmt.Sprint(day))
		check(e)
		if !exists {
			return
		}

		dir := path.Join(year, fmt.Sprint(day))
		os.MkdirAll(dir, os.ModePerm)
		filePath := path.Join(dir, "input.txt")

		if _, err := os.Stat(filePath); errors.Is(err, os.ErrNotExist) {
			e = os.WriteFile(filePath, input, 0777)
			check(e)
			fmt.Printf("Added input for %s\n", dir)
		} else {
			fmt.Printf("Input for %s is already there\n", dir)
		}

		day++
	}

}

func getInput(year string, day string) (input []byte, exists bool, e error) {
	url := fmt.Sprintf("https://adventofcode.com/%s/day/%s/input", year, day)
	request, e := http.NewRequest("GET", url, nil)
	check(e)
	request.AddCookie(&http.Cookie{
		Name:  "session",
		Value: getSessionCookie(),
	})

	response, e := http.DefaultClient.Do(request)
	if e != nil {
		return
	}
	defer response.Body.Close()

	if response.StatusCode == 200 {
		body, e := io.ReadAll(response.Body)
		if e != nil {
			return []byte{}, false, e
		}
		input = body
		exists = true

	} else if response.StatusCode == 404 {
		exists = false
	} else {
		e = fmt.Errorf("Unable to retrieve input. Error: " + response.Status)
	}

	return
}

func getSessionCookie() string {
	cookie, e := os.Open("cookie.txt")
	check(e)
	defer cookie.Close()
	cookieContents, e := io.ReadAll(cookie)
	check(e)
	return strings.TrimSpace(string(cookieContents))
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
