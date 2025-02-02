package receivingchain

import (
	"fmt"

	"github.com/rylenko/bastion/pkg/ratchet/keys"
)

type ReceivingChain struct {
	masterKey         *keys.MessageMaster
	headerKey         *keys.Header
	nextHeaderKey     *keys.Header
	nextMessageNumber uint32
	config            *Config
}

func New(nextHeaderKey *keys.Header, config *Config) *ReceivingChain {
	return &ReceivingChain{
		masterKey:         nil,
		headerKey:         nil,
		nextHeaderKey:     nextHeaderKey,
		nextMessageNumber: 0,
		config:            config,
	}
}

func (chain *ReceivingChain) Advance() (*keys.Message, error) {
	if chain.config == nil {
		return nil, fmt.Errorf("%w: config is nil", ErrInvalidValue)
	}

	if chain.config.crypto == nil {
		return nil, fmt.Errorf("%w: config crypto is nil", ErrInvalidValue)
	}

	newMasterKey, messageKey, err := chain.config.crypto.AdvanceChain(chain.masterKey)
	if err != nil {
		return nil, fmt.Errorf("%w: advance via crypto: %w", ErrCrypto, err)
	}

	chain.masterKey = newMasterKey
	chain.nextMessageNumber++

	return messageKey, nil
}

func (chain *ReceivingChain) Upgrade(masterKey *keys.MessageMaster, nextHeaderKey *keys.Header) {
	chain.masterKey = masterKey
	chain.headerKey = chain.nextHeaderKey
	chain.nextHeaderKey = nextHeaderKey
	chain.nextMessageNumber = 0
}
