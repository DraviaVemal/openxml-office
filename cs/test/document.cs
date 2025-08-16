// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using draviavemal.openxml_office.global_2007;
using draviavemal.openxml_office.document_2007;

namespace openxmloffice.tests
{
    /// <summary>
    /// Word Test
    /// </summary>
    [TestClass]
    public class Document
    {
        private static readonly string resultPath = "../../test_results";
        /// <summary>
        /// Initialize word Test
        /// </summary>
        /// <param name="context">
        /// </param>
        [ClassInitialize]
        public static void ClassInitialize(TestContext context)
        {
            if (!Directory.Exists(resultPath))
            {
                Directory.CreateDirectory(resultPath);
            }
            PrivacyProperties.ShareComponentRelatedDetails = false;
            PrivacyProperties.ShareIpGeoLocation = false;
            PrivacyProperties.ShareOsDetails = false;
            PrivacyProperties.SharePackageRelatedDetails = false;
            PrivacyProperties.ShareUsageCounterDetails = false;
        }
        /// <summary>
        /// Save the Test File After execution
        /// </summary>
        [ClassCleanup]
        public static void ClassCleanup()
        {
            // word.SaveAs(string.Format("{1}/test-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }

        /// <summary>
        /// 
        /// </summary>
        // [TestMethod]
        public void BlankFile()
        {
            Word word2 = new(new()
            {
                IsInMemory = false
            });
            word2.SaveAs(string.Format("{1}/Blank-{0}.docx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }

        /// <summary>
        /// Test existing file
        /// </summary>
        [TestMethod]
        public void OpenExistingWord()
        {
            Word word1 = new("test_files/basic_test.docx", new()
            {
                IsInMemory = false
            });
            word1.SaveAs(string.Format("{1}/EditStyle-{0}.docx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
            Assert.IsTrue(true);
        }
    }
}
