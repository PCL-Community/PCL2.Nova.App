export namespace account {
	
	export class AccountType {
	    type: string;
	    name: string;
	    uuid: string;
	    access_token?: string;
	    refresh_token?: string;
	    client_token?: string;
	    server?: string;
	    base_code?: string;
	    head_skin: string;
	
	    static createFrom(source: any = {}) {
	        return new AccountType(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.type = source["type"];
	        this.name = source["name"];
	        this.uuid = source["uuid"];
	        this.access_token = source["access_token"];
	        this.refresh_token = source["refresh_token"];
	        this.client_token = source["client_token"];
	        this.server = source["server"];
	        this.base_code = source["base_code"];
	        this.head_skin = source["head_skin"];
	    }
	}
	export class AccountList {
	    accounts: AccountType[];
	
	    static createFrom(source: any = {}) {
	        return new AccountList(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.accounts = this.convertValues(source["accounts"], AccountType);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace config {
	
	export class ProfileFolder {
	    Name: string;
	    AbsPath: string;
	
	    static createFrom(source: any = {}) {
	        return new ProfileFolder(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.Name = source["Name"];
	        this.AbsPath = source["AbsPath"];
	    }
	}
	export class ThemeConfig {
	    Name: string;
	    Mode: string;
	
	    static createFrom(source: any = {}) {
	        return new ThemeConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.Name = source["Name"];
	        this.Mode = source["Mode"];
	    }
	}
	export class CustomizeConfig {
	    Theme: ThemeConfig;
	
	    static createFrom(source: any = {}) {
	        return new CustomizeConfig(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.Theme = this.convertValues(source["Theme"], ThemeConfig);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class Config {
	    Customize: CustomizeConfig;
	    ProfileFolder: ProfileFolder[];
	
	    static createFrom(source: any = {}) {
	        return new Config(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.Customize = this.convertValues(source["Customize"], CustomizeConfig);
	        this.ProfileFolder = this.convertValues(source["ProfileFolder"], ProfileFolder);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	
	

}

