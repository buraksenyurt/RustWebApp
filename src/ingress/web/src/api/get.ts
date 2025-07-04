import {getCall} from "./utils";
import {Url} from "./url";
import {WorkItems} from "../interfaces/workItems";

export default async function getAll() {
    try {
        return await getCall<WorkItems>(
            new Url().getAll, 200
        );
    } catch (e) {
        console.error("Exception in getAll:", e);
        return {status: 500, error: "getAll failed"};
    }
}