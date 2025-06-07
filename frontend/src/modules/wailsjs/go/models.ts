export namespace launcher {
	
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

