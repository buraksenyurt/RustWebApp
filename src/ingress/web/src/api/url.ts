export class Url {
    baseUrl: string;
    getAll: string;

    constructor() {
        this.baseUrl = Url.getBaseUrl();
        this.getAll = `${this.baseUrl}/api/`;
    }

    static getBaseUrl(): string {
        let url = window.location.href;
        if (url.includes("http://localhost:3000/")) {
            return "http://0.0.0.0:8001/";
        }
        return window.location.href;
    }

}