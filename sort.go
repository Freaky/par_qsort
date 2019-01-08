package main
import (
    "fmt"
    "time"
    "math/rand"
    "sync"
)

func InsertSort(d[] int) {

    i := 1
    for i < len(d) {
        h := d[i]
        j := i - 1
        for j >= 0 && h < d[j] {
            d[j + 1] = d[j]
            j -= 1
        }
        d[j + 1] = h
        i += 1
    }
}

func Partition(d[] int) int {

    d[len(d) / 2], d[0] = d[0], d[len(d) / 2]
    piv := d[0]
    mid := 0
    i := 1
    for i < len(d) {
        if d[i] < piv {
            mid += 1
            d[i], d[mid] = d[mid], d[i]
        }
        i += 1
    }
    d[0], d[mid] = d[mid], d[0]
    return mid
}

/*
func QSort(d[] int) {

    for len(d) >= 30 {
        mid := Partition(d)
        if mid < len(d) / 2 {
            QSort(d[:mid])
            d = d[mid + 1:]
        } else {
            QSort(d[mid + 1:])
            d = d[:mid]
        }
    }

    InsertSort(d)
}
*/
func RandomInit(d[] int) {

    for i := 0; i < len(d); i++ {
        d[i] = rand.Intn(1000000000)
    }
}

func TestSorted(d[] int) {

    for i := 1; i < len(d); i++ {
        if d[i] < d[i - 1] {
            fmt.Println("Data is not sorted")
            break
        }
    }
}

func QuSort(d[] int, wg *sync.WaitGroup) {

    for len(d) >= 30 {
        mid := Partition(d)
        var dr[] int
        if mid < len(d) / 2 {
            dr = d[:mid]
            d = d[mid + 1:]
        } else {
            dr = d[mid + 1:]
            d = d[:mid]
        }
        if (len(dr) > 100000) {
            wg.Add(1)
            go func(d[] int) {
                QuSort(d, wg)
                wg.Done()
            }(dr)

        } else {
            QuSort(dr, wg)
        }
    }
    InsertSort(d)
}

func QSort(d[] int) {
    var wg sync.WaitGroup
    QuSort(d, &wg)
    wg.Wait()
}

func main() {

    const LEN = 50000000
    var data[LEN] int
    RandomInit(data[:])
    fmt.Printf("Sorting %d million numbers with Goroutines in Go ...\n",
        LEN / 1000000)
    start := time.Now()
    QSort(data[:])
    fmt.Printf("Time: %.3fs\n", time.Since(start).Seconds())
    TestSorted(data[:])
}

