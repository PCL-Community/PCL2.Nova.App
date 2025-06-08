package account

type AccountType struct {
	AType        string  `json:"type"`
	Name         string  `json:"name"`
	UUID         string  `json:"uuid"`
	AccessToken  *string `json:"access_token,omitempty"`
	RefreshToken *string `json:"refresh_token,omitempty"`
	ClientToken  *string `json:"client_token,omitempty"`
	Server       *string `json:"server,omitempty"`
	BaseCode     *string `json:"base_code,omitempty"`
	HeadSkin     string  `json:"head_skin"`
}

type AccountList struct {
	Accounts []AccountType `json:"accounts"`
}

type AccountBinding struct{}
