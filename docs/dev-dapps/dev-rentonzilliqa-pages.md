---
id: dev-rentonzilliqa-pages
title: Pages
keywords:
  - react
  - rentonzilliqa
  - pages
  - frontend
description: Building the Pages for the RentOnZilliqa frontend application
---

---

In this section, we will build the pages for the frontend application.

## App Component

We start with the `App` component.

We create the routes for our pages using
[`react-router-dom`](https://www.npmjs.com/package/react-router-dom).

We setup the `Toaster` from [`react-hot-toast`](https://react-hot-toast.com).

With the `useEffect` hook, we set up the following:

- We check if ZilPay is available on the browser and store it in context using
  `setZilPay`. If ZilPay is not available, an error is conveyed.
- We fetch the state of the contract and store it in context using `setContract`
- Subscriptions are set up which allow us to
  - Update the contract state and block number when there is a block update
    using
    [`zilPay.wallet.observableBlock`](https://zilpay.github.io/zilpay-docs/zilliqa-provider/#methods)
  - Update the ZilPay account when it is changed using
    [`zilPay.wallet.observableAccount`](https://zilpay.github.io/zilpay-docs/zilliqa-provider/#methods)
