package keys

type Message struct {
	bytes []byte
}

func NewMessage(bytes []byte) *Message {
	return &Message{bytes: bytes}
}
