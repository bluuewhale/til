package main

import (
	"fmt"
	"image"
	"image/color"
	"io"
	"os"
	"strings"
	"time"

	"golang.org/x/tour/reader"
)

type Summable interface {
	Sum() float64
}

func Sum(summable Summable) float64 {
	return summable.Sum()
}

type Vertex struct {
	X, Y float64
}

func (v Vertex) Sum() float64 {
	return v.X + v.Y
}

func (v *Vertex) Scale(f float64) {
	// reference receiver is needed when making modification to original value
	v.X = v.X * f
	v.Y = v.Y * f
}

func describe(s Summable) {
	fmt.Printf("(%v, %T)\n", s, s)
}

func doTypeSwitch(i interface{}) {
	switch v := i.(type) {
	case int:
		fmt.Println("int!")
	case string:
		fmt.Println("string!")
	default:
		fmt.Printf("%T! \n", v)
	}
}

type IPAddr [4]byte

func (ip IPAddr) String() string {
	return fmt.Sprintf("%d.%d.%d.%d", ip[0], ip[1], ip[2], ip[3])
}

func exerciseStringer() {
	hosts := map[string]IPAddr{
		"loopback":  {127, 0, 0, 1},
		"googleDNS": {8, 8, 8, 8},
	}
	for name, ip := range hosts {
		fmt.Printf("%v: %v\n", name, ip)
	}
}

// Errors
type MyError struct {
	When time.Time
	What string
}

func (e *MyError) Error() string {
	return fmt.Sprintf("%s, at %v", e.What, e.When)
}

func run() error {
	return &MyError{
		When: time.Now(),
		What: "runtime error",
	}
}

func testError() {
	if result := run(); result != nil {
		fmt.Println(result)
	}
}

type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
	v := float64(e)
	return fmt.Sprintf("cannot perform Sqrt() for negative number: %v", v)
}

func Sqrt(x float64) (float64, error) {
	if x < 0 {
		return 0, ErrNegativeSqrt(-1)
	}

	z := x
	z -= (z*z - x) / (2 * z)
	return z, nil
}

func exerciseError() {
	fmt.Println(Sqrt(2))
	fmt.Println(Sqrt(-2))
}

func testReader() {
	reader := strings.NewReader("Hello, Reader!")

	buf := make([]byte, 8) // buffer
	for {
		n, err := reader.Read(buf)
		fmt.Printf("n=%v, error=%v, buffer=%v\n", n, err, buf)
		fmt.Printf("buffer[:n]=%q\n", buf[:n])
		if err == io.EOF {
			break
		}
	}
}

type MyReader struct{}

func (reader MyReader) Read(buf []byte) (int, error) {
	var n int = len(buf)
	var char rune = 'A'
	var ascii byte = byte(char)

	for i := 0; i < n; i++ {
		buf[i] = ascii
	}
	return n, nil
}

func exerciseReader() {
	reader.Validate(MyReader{})
}

type Rot13Reader struct {
	r io.Reader
}

func rot13(b byte) byte {
	if b < 65 {
		return b
	} else if b < 78 {
		return b + 13
	} else if b < 91 {
		return b - 13
	} else if b < 97 {
		return b
	} else if b < 110 {
		return b + 13
	} else if b < 123 {
		return b - 13
	}
	return b
}

func (reader Rot13Reader) Read(buf []byte) (int, error) {
	n, err := reader.r.Read(buf)

	if err == nil {
		for i := 0; i < n; i++ {
			buf[i] = rot13(buf[i])
		}
	}

	return n, err
}

func exerciseRot13Reader() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := Rot13Reader{s}
	io.Copy(os.Stdout, &r)
}

// Images
type MyImage struct {
	Inner [][]uint8
	dx    int
	dy    int
}

func newMyImage(dx, dy int) MyImage {

	var inner [][]uint8
	img := MyImage{inner, dx, dy}

	for x := 0; x < dx; x++ {
		row := make([]uint8, dy)
		img.Inner = append(img.Inner, row)

		for y := 0; y < dy; y++ {
			img.Inner[x][y] = uint8((x + y) / 2)
		}
	}

	return img
}

func (img MyImage) ColorModel() color.Model {
	// ColorModel returns the Image's color model.
	return color.RGBAModel
}

func (img MyImage) Bounds() image.Rectangle {
	// ColorModel returns the Image's color model.
	return image.Rect(0, 0, img.dx, img.dy)
}

func (img MyImage) At(x, y int) color.Color {
	// ColorModel returns the Image's color model.
	return color.RGBA{}
}

func execiseImages() {

}

func main() {
	v := Vertex{X: 3, Y: 4}
	fmt.Println(v.Sum())

	v.Scale(10) // pointer indirection => (&v).Scale(10)
	p := &v
	fmt.Println(p.Sum()) // pointer indirection => (*p).Scale(10)

	var summable Summable = &v
	fmt.Println(Sum(summable))

	describe(summable)

	// empty interface may hold values of any type
	var a int = 1
	var b string = "hi"
	var c float64 = 1.5
	doTypeSwitch(a)
	doTypeSwitch(b)
	doTypeSwitch(c)

	// exercise
	exerciseStringer()

	// Errors
	testError()
	exerciseError()

	// Reader
	testReader()
	exerciseReader()
	exerciseRot13Reader()
}
