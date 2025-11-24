package errors

import (
	"github.com/dracondev/go-templ-htmx-ex/libs/httperrx"
)

// Re-export httperrx types and functions for backward compatibility
type AppError = httperrx.AppError

var (
	NewBadRequestError      = httperrx.NewBadRequestError
	NewUnauthorizedError    = httperrx.NewUnauthorizedError
	NewForbiddenError       = httperrx.NewForbiddenError
	NewNotFoundError        = httperrx.NewNotFoundError
	NewInternalServerError  = httperrx.NewInternalServerError
	NewError                = httperrx.NewError
	WriteError              = httperrx.WriteError
	ErrorHandler            = httperrx.ErrorHandler
)
