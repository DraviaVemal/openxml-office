macro_rules! impl_xml_part_lifecycle {
    ($part:ty) => {
        impl Drop for $part {
            fn drop(&mut self) {
                let _ =
                    <$part as $crate::global::traits::XmlDocumentPartClose>::close_document(self);
            }
        }

        impl $crate::global::traits::XmlDocumentPartFlush for $part {}
    };
}

pub(crate) use impl_xml_part_lifecycle;
