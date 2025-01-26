package chains

import (
	"fmt"

	"github.com/rylenko/sapphire/pkg/shield/keys"
)

// Sending is the sending chain, which is responsible for encrypting the messages being sent. The sending chain of the
// sender is equal to the receiving chain of the recipient.
type Sending struct {
	masterKey                         *keys.MessageMaster
	headerKey                         *keys.Header
	nextHeaderKey                     *keys.Header
	nextMessageNumber                 uint32
	previousSendingChainMessagesCount uint32
}

// NewSending creates a new sending chain.
func NewSending(masterKey *keys.MessageMaster, headerKey, nextHeaderKey *keys.Header) *Sending {
	return &Sending{
		masterKey:                         masterKey,
		headerKey:                         headerKey,
		nextHeaderKey:                     nextHeaderKey,
		nextMessageNumber:                 0,
		previousSendingChainMessagesCount: 0,
	}
}

// Forward moves the sending chain forward. In other words, creating a new message master key and a new message key.
//
// This method is a wrapper around the provider that sets a new message master key into the current chain.
func (chain *Sending) Forward(provider Provider) (*keys.Message, error) {
	if provider == nil {
		return nil, fmt.Errorf("%w: provider is nil", ErrInvalidValue)
	}

	newMasterKey, messageKey, err := provider.ForwardMessageChain(chain.masterKey)
	if err != nil {
		return nil, fmt.Errorf("%w: forward message chain: %w", ErrProvider, err)
	}

	chain.masterKey = newMasterKey
	chain.nextMessageNumber++

	return messageKey, nil
}
