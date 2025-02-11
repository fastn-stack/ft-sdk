pub struct EmailBind {
    pub from_name: ft_sys_shared::SqliteRawValue,
    pub from_address: ft_sys_shared::SqliteRawValue,
    pub to_address: ft_sys_shared::SqliteRawValue,
    pub subject: ft_sys_shared::SqliteRawValue,
    pub body_html: ft_sys_shared::SqliteRawValue,
    pub body_text: ft_sys_shared::SqliteRawValue,
    pub reply_to: ft_sys_shared::SqliteRawValue,
    pub cc_address: ft_sys_shared::SqliteRawValue,
    pub bcc_address: ft_sys_shared::SqliteRawValue,
    pub mkind: ft_sys_shared::SqliteRawValue,
}

impl ft_sys_shared::Email {
    pub fn to_bind(self) -> EmailBind {
        let rendered = match self.content {
            ft_sys_shared::EmailContent::Rendered(rendered) => rendered,
            ft_sys_shared::EmailContent::FromMKind { .. } => {
                unreachable!("must be pre-rendered")
            }
        };

        EmailBind {
            from_name: ft_sys_shared::SqliteRawValue::Text(self.from.name.unwrap_or_default()),
            from_address: ft_sys_shared::SqliteRawValue::Text(self.from.email),
            to_address: ft_sys_shared::SqliteRawValue::Text(to_comma_separated_str(self.to)),
            subject: ft_sys_shared::SqliteRawValue::Text(rendered.subject),
            body_html: ft_sys_shared::SqliteRawValue::Text(rendered.body_html),
            body_text: ft_sys_shared::SqliteRawValue::Text(rendered.body_text),
            reply_to: self
                .reply_to
                .map(to_comma_separated_str)
                .map(ft_sys_shared::SqliteRawValue::Text)
                .unwrap_or(ft_sys_shared::SqliteRawValue::Null),
            cc_address: ft_sys_shared::SqliteRawValue::Text(to_comma_separated_str(self.cc)),
            bcc_address: ft_sys_shared::SqliteRawValue::Text(to_comma_separated_str(self.bcc)),
            mkind: ft_sys_shared::SqliteRawValue::Text(self.mkind),
        }
    }
}

fn to_comma_separated_str<const N: usize>(
    x: smallvec::SmallVec<ft_sys_shared::EmailAddress, N>,
) -> String {
    x.into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn to_comma_separated_str() {
//         assert_eq!(
//             super::to_comma_separated_str(vec![ft_sys_shared::EmailAddress::"Alice", "alice@a.com")]),
//             "Alice <alice@a.com>"
//         );
//         assert_eq!(
//             super::to_comma_separated_str(vec![("Alice", "alice@a.com"), ("Bob", "bob@a.com")]),
//             "Alice <alice@a.com>, Bob <bob@a.com>"
//         );
//     }
// }
