impl ft_sys_shared::Email {
    pub fn to_bind(
        self,
        _rendered: ft_sys_shared::RenderedEmail,
    ) -> [ft_sys_shared::SqliteRawValue; 6] {
        let to = to_comma_separated_str(self.to);
        let reply_to = self.reply_to.map(to_comma_separated_str);
        let cc = to_comma_separated_str(self.cc);
        let bcc = to_comma_separated_str(self.bcc);

        todo!()
        // [to, reply_to, cc, bcc, self.mkind, self.content]
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
