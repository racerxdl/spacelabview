package api

type SpaceError string

func (a SpaceError) String() string {
	return string(a)
}

func (a SpaceError) Error() string {
	return string(a)
}

const InvalidSecretError = SpaceError("Invalid Secret")
const AccessDeniedError = SpaceError("Access Denied")
