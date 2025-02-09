package receivingchain

import (
	"fmt"

	"github.com/rylenko/bastion/pkg/ratchet/errors"
	"github.com/rylenko/bastion/pkg/ratchet/keys"
)

type ReceivingChain struct {
	masterKey         *keys.MessageMaster
	headerKey         *keys.Header
	nextHeaderKey     *keys.Header
	nextMessageNumber uint64
	config            *Config
}

func New(
	masterKey *keys.MessageMaster,
	headerKey *keys.Header,
	nextHeaderKey *keys.Header,
	nextMessageNumber uint64,
	config *Config,
) *ReceivingChain {
	return &ReceivingChain{
		masterKey:         masterKey,
		headerKey:         headerKey,
		nextHeaderKey:     nextHeaderKey,
		nextMessageNumber: nextMessageNumber,
		config:            config,
	}
}

func (rc *ReceivingChain) Advance() (*keys.Message, error) {
	if rc.config == nil {
		return nil, fmt.Errorf("%w: config is nil", errors.ErrInvalidValue)
	}

	if rc.config.crypto == nil {
		return nil, fmt.Errorf("%w: config crypto is nil", errors.ErrInvalidValue)
	}

	newMasterKey, messageKey, err := rc.config.crypto.AdvanceChain(rc.masterKey)
	if err != nil {
		return nil, fmt.Errorf("%w: advance via crypto: %w", errors.ErrCrypto, err)
	}

	rc.masterKey = newMasterKey
	rc.nextMessageNumber++

	return messageKey, nil
}

func (rc *ReceivingChain) Clone() *ReceivingChain {
	if rc == nil {
		return nil
	}

	return New(rc.masterKey.Clone(), rc.headerKey.Clone(), rc.nextHeaderKey.Clone(), rc.nextMessageNumber, rc.config)
}

func (rc *ReceivingChain) Upgrade(masterKey *keys.MessageMaster, nextHeaderKey *keys.Header) {
	rc.masterKey = masterKey
	rc.headerKey = rc.nextHeaderKey
	rc.nextHeaderKey = nextHeaderKey
	rc.nextMessageNumber = 0
}
