#![feature(impl_trait_projections)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use anyhow::Result;
use queries::{
    content_my_ticket::ContentMyTicketReq,
    static_landing_ranking_section::{
        StaticLandingRankingSectionParam, StaticLandingRankingSectionReq,
    },
    Query,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub mod endpoints;
pub mod queries;
pub mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let mut header_map = HeaderMap::new();
    header_map.insert("cookie",  HeaderValue::from_static("_kpwtkn=U2FsdGVkX19n11nypyTDL3BA1d0y1KSGgyRrSdSwHTetL5Y0zCnmrN%2FxOPGFkOmvSm0THNCPSr1QZvj7o%2FsUwqJPqMqsw%2FDCUkLzRSY%2BsOuJ9UaQOzOwbvTiozEHCCcF2Zf8cHtDBLvPol43IWafcXqn9eBOglknIurWocSQKn7T2u7pI0euRN7fiM01w1lObdJ0fSzqyGpGs6uHcv59IRwj1rqVsPkE4KVcU%2FKDvWrJn7zTcJUIwhyTvLUAk01GNqOACSRiXCkLv9%2FLa%2FUHtoeeW%2F5NFtKEBYkH8OriosiudwCvVD5w1apPLGjQFSsPoMCaQLCvSn7mrwD%2Br23yLNYOqOOS%2F6Tg6ksU7zwGP863jgQAqoTHaKx6kys%2FVFk6nObYPShjT3R9ygWXzs3Kl8KPuJLAiy%2FGkZRVaKJv2oc%2BZkE96SlNzhgF7ZRjParZh9WiVzUbV%2FcdvtnMjKx6uvnEGKuid4AivE2JvysgQ6RqDNm%2BeoY%2FRQt%2FuMc2HnD4xqfSpzmOZyd%2F%2B4Si4Z9CMa7mtIaFWI1ez6%2FkMbhPeFKOFD2N5RmfWcNLWQny4hOot9wnzBmgZ93%2FYkY6uHzRGsVIAGbQn4hX%2FBwwGBZYIKPA1Rtvg398tktHCUNpd2uXL0hsJGiVj0kYhsIvHUT6ZFyONI0kDW61GGZuEx0nzWv%2FwQD3T4sDLHbp0xAb38GbecOgtaTF%2FOAZkq8Uo%2B68t5akM2xE8a0hD%2FrtBVNwyUCbpRcYt4tln3VO2ZFgERE6rOS4dShzKgxG2DUVX10BGz1XHK0VQoQlq12EdpITteC2RHFcpFg2xSs8lP9gmuGcCMwRdpwq38Je8qdmnLE2N%2FGk7p1NBlFGq3%2FNjmrqAji%2BtR0%2FdD99fDJq%2FNCbQxighiS98oj0MKcLvQlTDZpoT4gnT9ipFja%2FWBzEUA1dQHjBo7McXJEiKX987wOcQ49F3iod42RDkffoIxSpuBmtt6odrY1YY%2FRWscbP1TQ410SqWR5J3vFf4JGspVvPRv99DmDHZMHc%2F4hFETK3NErFd7%2F%2FCaVxaWRlW%2BBr3O%2FXsy0aGnQQiY0XBUTHOm7b1GjfPPItpssnQLoYzdLDBsWazK3VJAyE9cruAbMHkkAVZBKu%2BVaeR%2B%2BVtxc%2FFLroN%2FJt1%2FyVwPU7qq2LYlEv9Ihd4wUCz%2F%2FK07R6mU3l%2FFUFTz067yCk4IlmVpXbzcjhFtahNakqWxlEv7VNdiCjcLYWikeidgj%2BM6huDxYpqYx89%2BrHzlsL%2BJzUvO45KraBcj9BfPKLl0%2BaH8DS7iWtWdw8TYtsNfD8Kqgqcjzr5VdXN958qXO%2B%2F3O8hCVvHa0ZEKM6VQ3uC18%2FjEOPGiDIsRO4qctwAaVIP45PmyM8lRVx4WynwKDpcXqhMyBCR7V0XT5EuxaFwIVhvwfl%2B%2B6Od0lo3%2F9lum8woGKaP28%2B6q0%2FX%2BwE7aBG0X6NQZ%2FF3UQtJ9ztHQjxfYBNk5BXdGbDoMVSrEUSQbSUcG%2BU9jr58o2FvVQCld8IguIJ395H7g7MG9z%2BkL4paJq9cVHMQsMFbZwK7ZzPKBVsMddnGspUuaUdfwGEZXVw286%2FojdQVtxsR7Oymlv2Rc9ZkjJnQqOkFIxpnFcLUwQo0752wMURhnCoGRwabvDe%2FfZzDBlPAh2osfrrfHJrKF7QleOEhQSfFq%2BqFdNN8o%2Bq3smw2UO%2F4lAcf1e0YOwfkuSzS9hFmoOkFuhtGJhDsuO7%2FQSfvsWd8PKZEWQut4XvwuU5HUczPUYIMv17uEkx9xI%2FLVL9leJ0HXDPMbOsL4MgjSc6CqWCyyHM%2FiJ5F%2F7SH839DWyQ5BZfCCVaDXn5c89xPaaKRASWgBvN49AyHehwriR%2BDFgjnO%2B4%2BOaxIIPPVloDRsG7rQvhZZcz2vWqvLlVaqmwj%2B5LH6NwJmLCB0BP821UvX%2Ba1%2FGo69QiyUwc6krBsxqZMB52m1Q%3D"));
    let client = Client::builder()
        .default_headers(header_map)
        .user_agent("test")
        .build()?;
    for i in 0..100 {
        let res = StaticLandingRankingSectionReq {
            param: StaticLandingRankingSectionParam {
                category_uid: 0,
                page: i,
            },
        }
        .send(&client)
        .await?;
        for item in &res.static_landing_ranking_section.groups[0].items {
            for (key, val) in item.scheme.query_pairs() {
                if key == "series_id" {
                    println!("{} {}", item.row1, val);
                    let res = ContentMyTicketReq {
                        series_id: val.parse()?,
                    }
                    .send(&client)
                    .await?;
                    println!("{:#?}", res.content_my_ticket)
                }
            }
        }
    }
    Ok(())
}
