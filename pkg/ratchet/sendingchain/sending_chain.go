package sendingchain

import (
	"fmt"

	"github.com/rylenko/bastion/pkg/ratchet/errors"
	"github.com/rylenko/bastion/pkg/ratchet/keys"
)

type SendingChain struct {
	masterKey                         *keys.MessageMaster
	headerKey                         *keys.Header
	nextHeaderKey                     *keys.Header
	nextMessageNumber                 uint64
	previousSendingChainMessagesCount uint64
	config                            *Config
}

func New(
	masterKey *keys.MessageMaster,
	headerKey *keys.Header,
	nextHeaderKey *keys.Header,
	nextMessageNumber uint64,
	previousSendingChainMessagesCount uint64,
	config *Config,
) *SendingChain {
	return &SendingChain{
		masterKey:                         masterKey,
		headerKey:                         headerKey,
		nextHeaderKey:                     nextHeaderKey,
		nextMessageNumber:                 nextMessageNumber,
		previousSendingChainMessagesCount: previousSendingChainMessagesCount,
		config:                            config,
	}
}

func (sc *SendingChain) Advance() (*keys.Message, error) {
	if sc.config == nil {
		return nil, fmt.Errorf("%w: config is nil", errors.ErrInvalidValue)
	}

	if sc.config.crypto == nil {
		return nil, fmt.Errorf("%w: config crypto is nil", errors.ErrInvalidValue)
	}

	newMasterKey, messageKey, err := sc.config.crypto.AdvanceChain(sc.masterKey)
	if err != nil {
		return nil, fmt.Errorf("%w: advance via crypto: %w", errors.ErrCrypto, err)
	}

	sc.masterKey = newMasterKey
	sc.nextMessageNumber++

	return messageKey, nil
}

func (sc *SendingChain) Clone() *SendingChain {
	if sc == nil {
		return nil
	}

	return New(
		sc.masterKey.Clone(),
		sc.headerKey.Clone(),
		sc.nextHeaderKey.Clone(),
		sc.nextMessageNumber,
		sc.previousSendingChainMessagesCount,
		sc.config,
	)
}

func (sc *SendingChain) HeaderKey() *keys.Header {
	return sc.headerKey
}

func (sc *SendingChain) NextMessageNumber() uint64 {
	return sc.nextMessageNumber
}

func (sc *SendingChain) PreviousSendingChainMessagesCount() uint64 {
	return sc.previousSendingChainMessagesCount
}

func (sc *SendingChain) Upgrade(masterKey *keys.MessageMaster, nextHeaderKey *keys.Header) {
	sc.masterKey = masterKey
	sc.headerKey = sc.nextHeaderKey
	sc.nextHeaderKey = nextHeaderKey
	sc.previousSendingChainMessagesCount = sc.nextMessageNumber
	sc.nextMessageNumber = 0
}
