import axios, {AxiosResponse} from "axios";

export async function getCall<X>(
    url: string,
    expectedResponse: number) {
    let response = axios.get<X | string>(
        url,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "APPLICATION_TOKEN_1234"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}

async function handleRequest<T, X>(
    promise: Promise<AxiosResponse<X>>,
    expectedResponse: number) {
    let response: AxiosResponse<X>;
    try {
        response = await promise;
        if (response.status === expectedResponse) {
            return {
                status: response.status,
                data: response.data as X
            };
        } else {
            return {
                status: response.status,
                error: response.data as string
            };
        }
    } catch (error) {
        return {
            status: 500,
            error: 'Network or other error occurred'
        };
    }
}