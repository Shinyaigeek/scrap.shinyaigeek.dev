export abstract class RepositoryBase {
  protected readonly fetcher: (
    input: RequestInfo,
    init?: RequestInit | undefined
  ) => Promise<Response>;

  constructor(fetcher = fetch) {
    this.fetcher = fetch;
  }

  abstract read<T>(query?: Record<string, string>): Promise<T>;

  abstract write<T>(
    body?: Record<string, string>,
    query?: Record<string, string>
  ): Promise<T>;

  abstract update<T>(
    body?: Record<string, string>,
    query?: Record<string, string>
  ): Promise<T>;

  abstract delete<T>(query?: Record<string, string>): Promise<T>;
}
