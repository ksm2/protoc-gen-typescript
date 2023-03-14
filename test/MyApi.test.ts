import {
  BidiStreamingRequest,
  BidiStreamingResponse,
  ClientStreamingRequest,
  ClientStreamingResponse,
  GrpcService,
  MyApi,
  ServerStreamingRequest,
  ServerStreamingResponse,
  UnaryRequest,
  UnaryResponse,
} from '../gen';

describe('MyApi', () => {
  const service = mockGrpcService();
  const api = new MyApi(service);

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('should make a unary request', async () => {
    const req = new UnaryRequest();
    const res = new UnaryResponse();

    service.unary.mockResolvedValue(res);

    const returned = await api.unaryRpc(req);

    expect(returned).toBe(res);
  });

  it('should make a server-streaming request', async () => {
    const req = new ServerStreamingRequest();
    const res1 = new ServerStreamingResponse();
    const res2 = new ServerStreamingResponse();
    const res3 = new ServerStreamingResponse();

    service.serverStreaming.mockImplementation(async function* () {
      yield res1;
      yield res2;
      yield res3;
    });

    const returned = await toArray(api.serverStreamingRpc(req));

    expect(returned).toStrictEqual([res1, res2, res3]);
  });

  it('should make a client-streaming request', async () => {
    const req1 = new ClientStreamingRequest();
    const req2 = new ClientStreamingRequest();
    const req3 = new ClientStreamingRequest();
    const res = new ClientStreamingResponse();

    service.clientStreaming.mockResolvedValue(res);

    const returned = await api.clientStreamingRpc(toIter(req1, req2, req3));

    expect(returned).toBe(res);
  });

  it('should make a bidi-streaming request', async () => {
    const req1 = new BidiStreamingRequest();
    const req2 = new BidiStreamingRequest();
    const req3 = new BidiStreamingRequest();

    const res1 = new BidiStreamingResponse();
    const res2 = new BidiStreamingResponse();
    const res3 = new BidiStreamingResponse();

    service.bidiStreaming.mockImplementation(async function* () {
      yield res1;
      yield res2;
      yield res3;
    });

    const returned = await toArray(api.bidiStreamingRpc(toIter(req1, req2, req3)));

    expect(returned).toStrictEqual([res1, res2, res3]);
  });

  function mockGrpcService(): jest.Mocked<GrpcService> {
    return {
      unary: jest.fn(),
      serverStreaming: jest.fn(),
      clientStreaming: jest.fn(),
      bidiStreaming: jest.fn(),
    };
  }

  async function toArray<T>(iter: AsyncIterable<T>): Promise<readonly T[]> {
    const array = [];
    for await (const item of iter) {
      array.push(item);
    }
    return array;
  }

  async function* toIter<T>(...items: T[]): AsyncIterable<T> {
    yield* items;
  }
});
