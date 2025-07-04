import {getCall} from "./utils";
import {Url} from "./url";
import {WorkItems} from "../interfaces/workItems";

export default async function getAll() {
    let response = await getCall<WorkItems>(
        new Url().getAll, 200
    );
    return response;
}